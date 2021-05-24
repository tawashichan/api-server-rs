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
      throw new Error("env not found");
  }
})();
const app = new cdk.App();
new CfnStack(app, "api-server-rust-stack", conf);
