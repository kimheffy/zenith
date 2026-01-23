import {
  Body,
  Controller,
  Get,
  HttpCode,
  HttpStatus,
  Post,
  Request,
  UseGuards,
} from "@nestjs/common";
import type { UserCredentials } from "../user/user.type";
import { AuthGuard } from "./auth.guard";
import { SkipAuth } from "./auth.meta";
// biome-ignore lint/style/useImportType: need this for DI
import { AuthService } from "./auth.service";
import { DatabaseOperationException } from "../errors/common";

@Controller("auth")
export class AuthController {
  constructor(private authService: AuthService) {}

  @SkipAuth()
  @Post("register")
  async register(@Body() registerDto: UserCredentials) {
    try {
      await this.authService.register(registerDto);
    } catch (e) {
      console.log("register error... ", e);
      throw new DatabaseOperationException(e.message);
    }
  }

  @SkipAuth()
  @HttpCode(HttpStatus.OK)
  @Post("login")
  signIn(
    @Body() signInDto: UserCredentials,
  ): Promise<{ access_token: string }> {
    return this.authService.signin(signInDto);
  }

  @UseGuards(AuthGuard)
  @Get("profile")
  getProfile(@Request() req) {
    return req.user;
  }
}
