# ICS Subscribe

从 TOML 配置文件生成 iCalendar (.ics) 订阅源，通过 HTTP 提供日历订阅服务。

## 功能

- 从 TOML 配置文件读取事件定义
- 生成符合 RFC 5545 标准的 .ics 日历文件
- 支持 RRULE 重复规则
- 支持 VALARM 提醒（事件开始前 N 分钟）
- 通过 Axum HTTP 服务提供日历订阅

## 快速开始

```bash
# 构建
cargo build --release

# 使用默认配置启动
./target/release/ics-subscribe

# 指定配置文件和端口
CONFIG_PATH=config_34082301.toml PORT=8080 ./target/release/ics-subscribe
```

服务启动后，在日历客户端（Apple 日历、Google Calendar 等）中订阅以下地址：

```
http://localhost:3000/calendar.ics
```

## 配置格式

配置文件使用 TOML 格式，示例：

```toml
calendar_name = "我的日历"

[[events]]
title = "团队站会"
start_time = "2026-05-15T09:00:00+08:00"
end_time = "2026-05-15T09:30:00+08:00"
location = "会议室A"
description = "每日团队站会"
reminder = 15

[[events]]
title = "全员大会"
start_time = "2026-06-01T14:00:00+08:00"
end_time = "2026-06-01T15:30:00+08:00"
location = "大礼堂"
rrule = "FREQ=MONTHLY;COUNT=6"
reminder = 30
```

### 事件字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `title` | string | 是 | 事件标题 |
| `start_time` | string | 是 | 开始时间，RFC 3339 格式 |
| `end_time` | string | 否 | 结束时间，RFC 3339 格式 |
| `location` | string | 否 | 地点 |
| `description` | string | 否 | 描述 |
| `rrule` | string | 否 | RRULE 重复规则，如 `FREQ=WEEKLY;COUNT=10` |
| `reminder` | integer | 否 | 提前提醒分钟数 |

时间格式支持带时区偏移的 RFC 3339，如 `+08:00` 表示中国标准时间。

## 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `CONFIG_PATH` | `config.toml` | 配置文件路径 |
| `PORT` | `3000` | 监听端口 |

## 项目结构

```
src/
  main.rs      # Axum HTTP 服务入口
  config.rs    # TOML 配置解析
  calendar.rs  # iCalendar 生成逻辑
```

## 依赖

- [icalendar](https://crates.io/crates/icalendar) — iCalendar 生成（含 recurrence 特性）
- [chrono](https://crates.io/crates/chrono) — 日期时间处理
- [toml](https://crates.io/crates/toml) — TOML 解析
- [axum](https://crates.io/crates/axum) — HTTP 框架
- [tokio](https://crates.io/crates/tokio) — 异步运行时

## License

MIT

---

本项目代码由 Codebuddy + GLM 5.1 完成。
