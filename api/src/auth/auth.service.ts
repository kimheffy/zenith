import { Injectable, UnauthorizedException } from "@nestjs/common";
// biome-ignore lint/style/useImportType: need this for DI
import { ConfigModule, ConfigService } from "@nestjs/config";
// biome-ignore lint/style/useImportType: need this for DI
import { JwtService } from "@nestjs/jwt";
import { compare, hash } from "bcrypt";
// biome-ignore lint/style/useImportType: need this for DI
import { UserService } from "../user/user.service";

import type { UserCredentials } from "../user/user.type";

@Injectable()
export class AuthService {
  constructor(
    private configService: ConfigService,
    private userService: UserService,
    private jwtService: JwtService,
  ) {}

  async register({ email, password }: UserCredentials) {
    await ConfigModule.envVariablesLoaded;

    const saltRounds = this.configService.get<number>("SALT_ROUNDS", {
      infer: true,
    });

    try {
      const saltHash = await hash(btoa(password), parseInt(saltRounds, 10));

      this.userService.registerUser({ email, password: saltHash });
    } catch (e) {
      console.error("failed to salt hash...", e);
    }
  }

  async signin(userCred: UserCredentials) {
    const foundUser = await this.userService.findByUserEmail(userCred.email);

    if (!foundUser) {
      throw new Error("Unable to find an account.");
    }

    const userDoesMatch = await compare(
      btoa(userCred.password),
      foundUser.password,
    );

    if (!userDoesMatch) {
      throw new UnauthorizedException();
    }

    const payload = { sub: foundUser.id, email: foundUser.email };

    return {
      access_token: await this.jwtService.signAsync(payload),
    };
  }
}
