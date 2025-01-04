import type { AxiosResponse } from "axios";
import { http } from "../../http";
import type { CreateUserInput, ListUserOutput } from "./types";

export class UserService {
  public async create(input: CreateUserInput) {
    return http.post('/create', input);
  }

  public async list(): Promise<AxiosResponse<ListUserOutput[]>> {
    return http.get('/list')
  }
}
