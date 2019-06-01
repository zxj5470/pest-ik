# pest-ik

这是一个基于 Rust 以及 pest 包的编译器前端项目……想要实现简化之后的 Kotlin 语法。
因此他应该是 Kotlin 语法的一个子集，但也许会添加部分不兼容特性。
之后应该会添加编译器后端……
(你看那个样例的文件名是 `sample.kt`)

> This is a compiler front-end project based on Rust and pest packages, which implements a simplified Kotlin syntax, 
> after which the compiler back end should be added.
> So it should be a subset of the Kotlin syntax, but it may add some incompatible features.
> And you'll see the sample file name is `sample.kt`.

# 关于名字
和唱跳Rap真的没啥关系……
  
# 暂时支持的语法
- [x] 定义
- [x] 赋值
- [x] 注释（单行、多行及嵌套）
- [x] +-*/
- [x] 中缀表达式
- [x] 函数调用（甚至还可以珂里化）（列表的解析还没做）

## 基本类型
- [x] Boolean (bool)
- [x] Float(f32/f64)
- [x] Int(i32)
- [x] 字符串（应该支持了转义字符）

## 特性
- [x] 支持一下字面量的类型推导

暂不支持其他任何复杂的语法23333
见 [sample.kt](sample.kt)

```kotlin
a=2
// 4.8
var `var`:String = "Spinn\"ing at the Boundary"


/*
another = "It's Escaped"/*
(2+1)*(2+2)
3 * 2 + 1

1 + 3 % 4*/*/

x = 100
y = "1"

flt = 2.33f
dbl = 4.66
//y = 1
```

# Result

```
[
    GlobalAssign {
        ident: "a",
        expr: Int(
            2
        )
    },
    GlobalDecl {
        modifier: "var",
        type_str: "String",
        ident: "var",
        expr: Str(
            "Spinn\\\"ing at the Boundary"
        )
    },
    GlobalAssign {
        ident: "x",
        expr: Int(
            100
        )
    },
    GlobalAssign {
        ident: "y",
        expr: Str(
            "1"
        )
    },
    GlobalAssign {
        ident: "flt",
        expr: Float(
            2.33
        )
    },
    GlobalAssign {
        ident: "dbl",
        expr: Double(
            4.66
        )
    }
]
```
