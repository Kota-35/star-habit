import { defineConfig } from "orval";

export default defineConfig({
  "star-habit": {
    input: "../packages/openapi/star-habit.json",
    output: {
      target: "./src/server/__generated__/endpoints.ts",
      schemas: "./src/server/__generated__/model",
      client: "react-query",
      httpClient: "axios",
      biome: true,
      override: {
        mutator: {
          path: "./src/server/mutator/custom-instance.ts",
          name: "customInstance",
        },
      },
    },
  },
});
