import * as cdk from "@aws-cdk/core";
import { Config } from "../config";
import * as ec2 from "@aws-cdk/aws-ec2";
import * as iam from "@aws-cdk/aws-iam";
import * as dynamo from "@aws-cdk/aws-dynamodb";
import * as ecr from "@aws-cdk/aws-ecr";
import * as apprunner from "@aws-cdk/aws-apprunner";
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

    const appRepository = new ecr.Repository(this, `${id}-repository`, {
      imageScanOnPush: false,
      repositoryName: `${id}-repository`,
    });

    const appBuildRole = new iam.Role(this, `${id}-build-role-id`, {
      roleName: `${id}-build-role`,
      assumedBy: new iam.ServicePrincipal("build.apprunner.amazonaws.com"),
    });

    const appTaskRole = new iam.Role(this, `${id}-task-role-id`, {
      roleName: `${id}-task-role`,
      assumedBy: new iam.ServicePrincipal("tasks.apprunner.amazonaws.com"),
    });

    appBuildRole.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "service-role/AWSAppRunnerServicePolicyForECRAccess"
      )
    );

    /*const service = new apprunner.CfnService(this, `${id}-apprunner-id`, {
      serviceName: `${id}-apprunner`,
      instanceConfiguration: {
        instanceRoleArn: appTaskRole.roleArn,
      },
      sourceConfiguration: {
        autoDeploymentsEnabled: true,
        imageRepository: {
          imageRepositoryType: "ECR",
          imageIdentifier: appRepository.repositoryUri + ":latest",
        },
      },
      autoScalingConfigurationArn: appBuildRole.roleArn,
    });*/

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

    userTable.addGlobalSecondaryIndex({
      indexName: "gsi_email",
      partitionKey: {
        name: "email",
        type: dynamo.AttributeType.STRING,
      },
    });

    const userEmailTable = new dynamo.Table(this, `${id}-user-email`, {
      tableName: `${id}-user-email-table`,
      partitionKey: {
        name: "email",
        type: dynamo.AttributeType.STRING,
      },
      billingMode: dynamo.BillingMode.PAY_PER_REQUEST,
      removalPolicy: RemovalPolicy.DESTROY,
    });
    userEmailTable.grantFullAccess(appTaskRole);
  }
}
