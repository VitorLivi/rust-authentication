import { http } from "../../http";
import type { SignUpInput } from "./types";

export class AuthService {
  public async login() { }

  public async logout() { }

  public async create(input: SignUpInput) {
    http.post('/create', input);
  }
}
