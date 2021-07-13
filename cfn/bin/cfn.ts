#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "@aws-cdk/core";
import { CfnStack } from "../lib/cfn-stack";
import { devConfig } from "../devConfig";

const conf = (() => {
  switch (process.env.NODE_ENV) {
    case "dev":
      return devConfig;
    default:
      return devConfig;
    //throw new Error("env not found");
  }
})();

const config = conf;
const appEnv = {
  account: config.account.id,
  region: config.account.region,
};

const app = new cdk.App();
new CfnStack(app, "api-server-rust-stack", conf, { env: appEnv });
