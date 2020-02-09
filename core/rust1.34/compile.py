#!/usr/bin/env python3
"""Rust Action Builder
#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
"""

from __future__ import print_function
import os, sys, codecs, subprocess
from os.path import abspath, exists, dirname
import time
import shutil

## utils
# write a file creating intermediate directories
def write_file(file, body):
    os.makedirs(dirname(file), mode=0o755, exist_ok=True)
    with open(file, mode="w", encoding="utf-8") as f:
        f.write(body)

# copy a file eventually replacing a substring
def copy_replace(src, dst, match=None, replacement=""):
    with codecs.open(src, 'r', 'utf-8') as s:
        body = s.read()
        if match:
            body = body.replace(match, replacement)
        write_file(dst, body)

## cargo
cargo_action = """[package]
name = "actions"
version = "0.1.0"
authors = ["Roberto Diaz <roberto@theagilemonkeys.com>"]
edition = "2018"

[dependencies]
serde_json = "1.0"
serde = "1.0"
serde_derive = "1.0"
"""

def build(tgt_dir):
     # support RELEASE
     cmd = ["cargo", "build"]
     bin_dir = "debug"
     if "RELEASE" in os.environ:
         cmd += "--release"
         bin_dir = "release"
     env = {
         "CARGO_HOME": "/usr/local/cargo",
         "PATH": "/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
         "RUSTUP_HOME": "/usr/local/rustup"
     }
     p = subprocess.Popen(cmd,
           stdout=subprocess.PIPE,
           stderr=subprocess.PIPE,
           cwd="/usr/src", env=env)
     (o, e) = p.communicate()
     if isinstance(o, bytes): o = o.decode('utf-8')
     if isinstance(e, bytes): e = e.decode('utf-8')
     if p.returncode != 0:
        sys.stdout.write(o)
        sys.stdout.write(e)
     else:
       shutil.move(
             "/usr/src/target/%s/action_loop" % bin_dir,
             "%s/exec" % tgt_dir)

def sources(main, src_dir):
    # move away the action dir and replace with the new
    tmpname = str(int(time.time()))
    shutil.move("/usr/src/actions", "/usr/src/src%s" % tmpname)
    shutil.move(src_dir, "/usr/src/actions")

    # move exec in the right place
    src_file = "/usr/src/actions/exec"
    if exists(src_file):
        os.makedirs("/usr/src/actions/src", mode=0o755, exist_ok=True)
        copy_replace(src_file, "/usr/src/actions/src/lib.rs")

    # add a cargo.toml if needed
    cargo_action_file = "/usr/src/actions/Cargo.toml"
    if not exists(cargo_action_file):
        write_file(cargo_action_file, cargo_action)

    # write the boilerplate in a temp dir
    launcher = "/usr/src/action_loop/tmp%s" % tmpname
    shutil.move("/usr/src/action_loop/src/main.rs", launcher)
    copy_replace(launcher, "/usr/src/action_loop/src/main.rs",
          "use actions::main as actionMain;",
          "use actions::%s as actionMain;" % main )

if __name__ == '__main__':
    if len(sys.argv) < 4:
        sys.stdout.write("usage: <main-function> <source-dir> <target-dir>\n")
        sys.stdout.flush()
        sys.exit(1)
    sources(sys.argv[1], abspath(sys.argv[2]))
    build(abspath(sys.argv[3]))
    sys.stdout.flush()
    sys.stderr.flush()
