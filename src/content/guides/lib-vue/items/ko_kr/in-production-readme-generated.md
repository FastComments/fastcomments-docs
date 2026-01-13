아마 콜백 등을 전달하는 경우 config을 인라인으로 정의하지 않는 것이 좋습니다. 대신, 당신은
config을 `computed` 블록에 정의하는 편이 좋습니다. 그렇지 않으면 콜백 등이 호출될 때마다 위젯 전체가 다시 렌더링됩니다.

[스피너 예제가 이를 수행하는 방법을 참조하세요.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)