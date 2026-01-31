You probably don't want to define the config inline if you're passing callbacks etc. Instead, you'll want to define
the config in a `computed` block, otherwise each time your callback etc is invoked the entire widget will re-render.

[See the spinner example for how to do this.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)