# sliding_window
滑动窗口

## db schema
```
CREATE TABLE IF NOT EXISTS kl_api_log (
    src_ip VARCHAR(64),
    session_id VARCHAR(255),
    api_addr VARCHAR(255)
    ts timestamp(6)
) WITH (OIDS = FALSE);

CREATE TABLE IF NOT EXISTS kl_src_ip_delta (
    src_ip VARCHAR(64),
    delta int4,
    ts timestamp(6)
) WITH (OIDS = FALSE);
```
## sql
```

```

## 思路
需要指定的变量：平均速度窗口，变化幅度
1. 从log中，计算平均速度（10秒平均），打点记录
2. 连续两点差值超过变化幅度，告警

## 数据量确认
ori_ip 去重数量 721875
session_id 去重数量 7285302
ori_ip + api_addr 去重数量 13534785
session_id + api_addr 去重数量 16773531

