您可能不想在内联中定义 config，特别是当您传入 callbacks 等时。相反，您会想要定义
config 在一个 `computed` 块中，否则每次调用您的 callbacks 等时，整个 widget 都会重新渲染。

[参见 spinner 示例了解如何执行此操作。](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)