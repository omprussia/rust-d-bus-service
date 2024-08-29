#!/usr/bin/env bash

cargo build --release
SOURCES_ROOT=`pwd` rpmbuild ./rpm/d-bus-service.spec -bb
