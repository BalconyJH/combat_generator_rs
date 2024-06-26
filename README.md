# CombatGenerator-rs

## WIP

## 环境

~~需要 `python` 环境~~

## 依赖

### ⚠️在.tools/CombatGenerator目录下执行所有命令

(如果已经安装过pipx和pdm可以跳过直接使用pdm install)

```shell
python -m pip install pipx
pipx ensurepath
```

重启终端

```shell
pipx install pdm
pdm install
```

## 使用

输入文件需预先保存到 `.tools/CombatGenerator/data/` 路径下,
产物如果没有使用可选参数将会使用默认值保存在 `.tools/CombatGenerator/data/output.json` 路径下

打开cmd, 执行以下命令

```shell
.venv\Scripts\Activate.ps1
cd src/combatgenerator
pdm run python __init__.py
```

## 可选参数

### 反序列器 将生成的combatJson反序列化为输入的结构体

    -r

### 传入路径

    -i path/to/yourfile.json(绝对路径)

### 输出路径

    -o path/to/yourfile.json(绝对路径)

### e.g.

例如我想使用反序列器，传入路径为F:/1.json, 输出路径为F:/2.json

    pdm run python __init__.py -r -i F:/1.json -o F:/2.json

## input说明

`"BasicATK": [
0,
30
]`中[0, 30]表示`pre_delay=0`, `post_delay=30`。
若不填则根据下表填入默认值

| Action         | Pre Delay | Post Delay |
|----------------|-----------|------------|
| Forward        | 0         | 50         |
| Backward       | 0         | 50         |
| Left           | 0         | 50         |
| Right          | 0         | 50         |
| LeftForward    | 0         | 50         |
| LeftBackward   | 0         | 50         |
| RightForward   | 0         | 50         |
| RightBackward  | 0         | 50         |
| BasicATK       | 0         | 200        |
| BasicATKCharge | 0         | 100        |
| Evade          | 0         | 150        |
| Ultimate       | 0         | 150        |
| UltimateCharge | 0         | 100        |
| WeaponSkill    | 0         | 150        |
| ELFSkill       | 0         | 150        |
| ExtraSkill     | 0         | 100        |
| QTE1           | 	0        | 	200       |
| QTE2           | 	0        | 	200       |

    {
        "mode": "[关卡类型]",
        "role": "[女武神名称]",
        "version": "[版本]",
        "combat": [
            "[Combat Action]",
            {"[Combat Action]": ["[pre delay]","[post delay]"]},
            "..."
        ]
    }

`prefix`: [mode]Combat[role]

    {
    "UniversalMirageCombatElysianPreheat": {
        "recognition": "Custom",
        "custom_recognizer": "Combating",
        "action": "Custom",
        "pre_delay": 1000,
        "post_delay": 1000,
        "next": [
            "UniversalMirageCombatElysianFinish",
            "UniversalMirageCombatElysian_001"
        ],
        "custom_action": "BasicATK"
    },
    "UniversalMirageCombatElysian_001": {
        "recognition": "Custom",
        "custom_recognizer": "Combating",
        "action": "Custom",
        "pre_delay": 0,
        "post_delay": 30,
        "next": [
            "UniversalMirageCombatElysianFinish",
            "UniversalMirageCombatElysian_002"
        ],
        "custom_action": "BasicATK"
    },
    "..."
    }

### 首项无论是什么延迟均为[1000, 1000]

具体样本可以去data下看

