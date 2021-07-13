import * as cdk from "@aws-cdk/core";
import { Config } from "../config";
import * as ec2 from "@aws-cdk/aws-ec2";
import * as iam from "@aws-cdk/aws-iam";
import * as dynamo from "@aws-cdk/aws-dynamodb";
import { RemovalPolicy } from "@aws-cdk/core";

export class CfnStack extends cdk.Stack {
  constructor(
    scope: cdk.Construct,
    id: string,
    config: Config,
    props?: cdk.StackProps
  ) {
    super(scope, id, props);

    const vpc = ec2.Vpc.fromLookup(this, `${id}-vpc-id`, {
      vpcId: config.vpc.id,
    });

    const appTaskRole = new iam.Role(this, `${id}-task-role-id`, {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
    });

    const appTaskExecutionRole = new iam.Role(
      this,
      `${id}-task-execution-role`,
      {
        assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
      }
    );

    const userTable = new dynamo.Table(this, `${id}-user`, {
      tableName: `${id}-user-table`,
      partitionKey: {
        name: "user_id",
        type: dynamo.AttributeType.STRING,
      },
      billingMode: dynamo.BillingMode.PAY_PER_REQUEST,
      removalPolicy: RemovalPolicy.DESTROY,
    });
    userTable.grantFullAccess(appTaskRole);

    // The code that defines your stack goes here
  }
}
