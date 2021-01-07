# Package

version       = "0.1.0"
author        = "José Antonio Córdoba Gómez"
description   = "A new awesome nimble package"
license       = "MIT"
srcDir        = "src"
binDir        = "bin"

bin            = @["jester_cards"]
# Dependencies

requires "nim >= 1.4.2"
requires "jester >= 0.5.0"
requires "dotenv >= 1.1.0"
requires "etcd_client >= 0.2.0"




task build, "Runs the test suite":
  exec "nim c -r  src/card.nim  src/app.nim"