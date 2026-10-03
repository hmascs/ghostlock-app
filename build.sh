#!/bin/sh
# GhostLock 一键构建（native 二进制 + 提取器 + host 测试）。
#
#   sh build.sh
#
# 依赖：make、C++23 编译器（g++ >= 13 / clang++ >= 17，或 Termux 的
#       aarch64-linux-android-clang++）、cargo（提取器用；离线构建需 ~/.cargo 有缓存）。
# 产物：build/native/ghostlock、build/extract/release/ghostlock-extract
#
# 说明：SHELL 显式传入，避免部分环境里 make 的 recipe shell 指到不存在的路径。
set -e
cd "$(dirname "$0")"

SHELL_BIN="$(command -v sh)"
echo "== [1/3] host 测试 =="
make -C src SHELL="$SHELL_BIN" native-host-tests

echo
echo "== [2/3] native 二进制 =="
make -C src SHELL="$SHELL_BIN" ghostlock

echo
echo "== [3/3] offsets 提取器 =="
if [ -f tools/extract_rs/Cargo.toml ]; then
  ( cd tools/extract_rs && (cargo build --release --offline 2>/dev/null || cargo build --release) )
fi

echo
echo "== 产物 =="
ls -la build/native/ghostlock 2>/dev/null || echo "  (native 未生成)"
ls -la build/extract/release/ghostlock-extract 2>/dev/null || echo "  (extractor 未生成)"
echo
echo "完成。APK 构建另见 docs/ 与 README（需 Android SDK + NDK + JDK17+）。"
