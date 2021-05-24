export interface Config {
  vpc: VPC;
  secrets: SecretParameters;
  env: Env;
  ecs: ECS;
  albCertificateArns: string[];
}

interface VPC {
  id: string;
}

interface SecretParameters {}

interface Env {
  USER_TABLE_NAME: string;
}

interface ECS {
  webServerTaskDef: TaskDef;
}

interface TaskDef {
  cpu: string;
  memoryMiB: string;
}
