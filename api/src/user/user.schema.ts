import { EntitySchema } from "typeorm";

export const UserSchema = new EntitySchema({
  name: "User",
  columns: {
    id: {
      type: Number,
      primary: true,
      generated: true,
    },
    email: {
      type: String,
    },
    password: {
      type: String,
    },
  },
});
