/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package runtime.actionContainers

import actionContainers.ActionContainer.withContainer
import actionContainers.{ActionContainer, BasicActionRunnerTests}
import common.WskActorSystem
import org.junit.runner.RunWith
import org.scalatest.junit.JUnitRunner

@RunWith(classOf[JUnitRunner])
class ActionLoopRustBasicTests extends BasicActionRunnerTests with WskActorSystem {

  val image = "actionloop-rust-v1.34"

  override def withActionContainer(env: Map[String, String] = Map.empty)(code: ActionContainer => Unit) = {
    withContainer(image, env)(code)
  }

  def withActionLoopContainer(code: ActionContainer => Unit) =
    withContainer(image)(code)

  behavior of image

  override val testNoSourceOrExec = TestConfig("")

  override val testNotReturningJson = TestConfig("", skipTest = true)

  override val testEcho =
    TestConfig("""|extern crate serde_json;
                  |use serde_json::{Error, Value};
                  |pub fn main(args: Value) -> Result<Value, Error> {
                  |    println!("hello stdout");
                  |    eprintln!("hello stderr");
                  |    Ok(args)
                  |}
                """.stripMargin)
  val snowman = """\""" + """u{2603}"""
  val space = """\""" + """u{0020}"""
  override val testUnicode =
    TestConfig(raw"""|extern crate serde_json;
            |use serde_derive::{Deserialize, Serialize};
            |use serde_json::{Error, Value};
            |#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            |struct Input {
            |    delimiter: String,
            |}
            |#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            |struct Output {
            |    winter: String,
            |}
            |pub fn main(args: Value) -> Result<Value, Error> {
            |    let input: Input = serde_json::from_value(args)?;
            |    let msg = format!("{} {} {}", input.delimiter,'☃',input.delimiter);
            |    println!("{}", msg);
            |    let output = Output {
            |        winter: msg,
            |   };
            |   serde_json::to_value(output)
            |}
        """.stripMargin)

  override val testEnv =
    TestConfig("""|extern crate serde_json;
                  |use serde_derive::{Deserialize, Serialize};
                  |use serde_json::{Error, Value};
                  |use std::env;
                  |use std::env::VarError;
                  |#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
                  |struct Output {
                  |    #[serde(skip_serializing_if = "Option::is_none")]
                  |    api_host:Option<String>,
                  |    #[serde(skip_serializing_if = "Option::is_none")]
                  |    api_key: Option<String>,
                  |    #[serde(skip_serializing_if = "Option::is_none")]
                  |    namespace: Option<String>,
                  |    #[serde(skip_serializing_if = "Option::is_none")]
                  |    action_name: Option<String>,
                  |    #[serde(skip_serializing_if = "Option::is_none")]
                  |    action_version: Option<String>,
                  |    #[serde(skip_serializing_if = "Option::is_none")]
                  |    activation_id: Option<String>,
                  |    #[serde(skip_serializing_if = "Option::is_none")]
                  |    deadline: Option<String>,
                  |}
                  |pub fn main(_args: Value) -> Result<Value, Error> {
                  |    let output = Output {
                  |        api_host: env::var("__OW_API_HOST").ok(),
                  |        api_key: env::var("__OW_API_KEY").ok(),
                  |        namespace: env::var("__OW_NAMESPACE").ok(),
                  |        action_name: env::var("__OW_ACTION_NAME").ok(),
                  |        action_version: env::var("__OW_ACTION_VERSION").ok(),
                  |        activation_id: env::var("__OW_ACTIVATION_ID").ok(),
                  |        deadline: env::var("__OW_DEADLINE").ok(),
                  |    };
                  |    serde_json::to_value(output)
                  |}
                """.stripMargin)

  override val testInitCannotBeCalledMoreThanOnce =
    TestConfig("""|extern crate serde_json;
                  |use serde_json::{Error, Value};
                  |pub fn main(args: Value) -> Result<Value, Error> {
                  |    Ok(args)
                  |}
                """.stripMargin)

  override val testEntryPointOtherThanMain =
    TestConfig(
      """|extern crate serde_json;
                  |use serde_json::{Error, Value};
                  |pub fn naim(args: Value) -> Result<Value, Error> {
                  |    Ok(args)
                  |}
                """.stripMargin,
      main = "naim")

  override val testLargeInput =
    TestConfig("""|extern crate serde_json;
                  |use serde_json::{Error, Value};
                  |pub fn main(args: Value) -> Result<Value, Error> {
                  |    Ok(args)
                  |}
                """.stripMargin)
}
