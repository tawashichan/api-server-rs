import { Config } from "./config";

export const devConfig: Config = {
  vpc: {
    id: "vpc-54c33c30",
  },
  secrets: {},
  env: {
    USER_TABLE_NAME: "user_tawashi",
  },
  ecs: {
    webServerTaskDef: {
      cpu: "1024",
      memoryMiB: "1024",
    },
  },
  albCertificateArns: [],
};
