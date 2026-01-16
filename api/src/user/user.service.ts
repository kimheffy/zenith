import { Injectable } from "@nestjs/common";
import { InjectRepository } from "@nestjs/typeorm";
// biome-ignore lint/style/useImportType: need this for DI
import { Repository } from "typeorm";
import { User } from "./user.entity";
import type { UserCredentials } from "./user.type";

@Injectable()
export class UserService {
  constructor(
    @InjectRepository(User)
    private userRepository: Repository<User>,
  ) {}

  async registerUser(registerUser: UserCredentials) {
    await this.userRepository.insert(registerUser);
  }

  findByUserEmail(email: string): Promise<User | null> {
    return this.userRepository.findOneBy({ email });
  }
}
