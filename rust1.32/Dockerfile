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
FROM actionloop/actionloop-v2:latest as builder
FROM rust:1.32
COPY --from=builder /bin/proxy /bin/proxy
RUN mkdir -p /action 
ADD compile /bin/compile
ADD src /usr/src
RUN cd /usr/src ; cargo build 
ENV OW_COMPILER=/bin/compile
WORKDIR /action
ENTRYPOINT ["/bin/proxy"]
