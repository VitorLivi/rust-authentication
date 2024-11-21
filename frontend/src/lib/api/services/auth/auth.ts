import { http } from "../../http";
import type { SignUpInput } from "./types";

export class AuthService {
  public async login() {}

  public async logout() { }

  public async signUp(input: SignUpInput) {
    http.post('/sign-up', input);
  }
}
