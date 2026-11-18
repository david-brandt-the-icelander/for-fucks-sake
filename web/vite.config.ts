import { defineConfig } from "vite";
// The release workflow sets FFS_VERSION from the git tag (e.g. "v0.2.0"); local builds say "dev".
const version = process.env.FFS_VERSION || "dev";

export default defineConfig({
  base: "./",
  define: { __FFS_VERSION__: JSON.stringify(version) },
  server: { port: 5173 },
  build: { target: "es2022" },
  test: { include: ["src/**/*.test.ts"] },
} as never);
