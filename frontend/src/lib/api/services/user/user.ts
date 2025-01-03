import { http } from "../../http";
import type { CreateUserInput } from "./types";

export class UserService {
  public async create(input: CreateUserInput) {
    return http.post('/create', input);
  }
}
