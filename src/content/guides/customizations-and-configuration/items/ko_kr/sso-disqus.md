Disqus와 FastComments Secure SSO 사이의 가장 큰 차이점은 Disqus는 암호화에 SHA1을 사용하는 반면, 우리는 SHA256을 사용한다는 점입니다.

이는 Disqus에서의 마이그레이션이 쉽다는 것을 의미합니다 - 사용되는 해싱 알고리즘을 SHA1에서 SHA256으로 변경하고 UI에 전달되는 속성 이름들을 업데이트하면 됩니다.