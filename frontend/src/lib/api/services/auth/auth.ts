import { http } from "../../http";
import type { RegisterInput } from "./types";

export class AuthService {
  public async login() {}

  public async logout() { }

  public async register(input: RegisterInput) {
    http.post('/register', input);
  }
}
