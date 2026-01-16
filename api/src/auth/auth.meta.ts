import { SetMetadata } from "@nestjs/common";

export const IS_PUBLIC_KEY = process.env.AUTH_META_PUBLIC_KEY;
export const SkipAuth = () => SetMetadata(IS_PUBLIC_KEY, true);
