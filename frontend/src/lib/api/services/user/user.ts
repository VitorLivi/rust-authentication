import type { AxiosResponse } from "axios";
import { http } from "../../http";
import type { CreateUserInput, FindUserByIdOutput, ListUserOutput, UpdateUserInput } from "./types";

export class UserService {
  public async create(input: CreateUserInput) {
    return http.post('/create', input);
  }

  public async update(input: UpdateUserInput) {
    return http.put('/update', input);
  }

  public async delete(id: string) {
    return http.delete(`/delete/${id}`);
  }

  public async list(): Promise<AxiosResponse<ListUserOutput[]>> {
    return http.get('/list')
  }

  public async findById(id: string): Promise<AxiosResponse<FindUserByIdOutput>> {
    return http.get(`/find-user/${id}`)
  }
}
