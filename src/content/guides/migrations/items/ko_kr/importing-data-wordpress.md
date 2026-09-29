Our [WordPress Plugin](https://wordpress.org/plugins/fastcomments/)은 강력한 UI 기반 가져오기 메커니즘을 제공합니다. 플러그인을 설치하면, WordPress 설치를 FastComments와 연결하고 기존 댓글 데이터를 복사하도록 안내합니다.

**이는 수동으로 복사하거나 다운로드하지 않고 수행됩니다.**

마이그레이션 과정은 UI를 통해 표시됩니다. 대부분의 마이그레이션은 몇 분밖에 걸리지 않습니다.

이 메커니즘은 마이그레이션 중에 WordPress 설치에 과도한 부하를 주지 않도록 설계되었습니다.

WordPress에서 사이트를 옮기는 경우, 플러그인을 사용하지 않고 WordPress XML 또는 CSV 내보내기를 가져올 수 있습니다. 자세히 보려면
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)를 참조하세요.

### CloudFlare & FireWalls

자동 WordPress 설정이 작동하려면 우리 측에서 귀하의 WordPress 설치에 호출을 해야 합니다. Cloudflare와 같은 방화벽이 이를 차단하면 통합이 실패할 수 있습니다. 이러한 경우, [우리는 귀하에게](https://fastcomments.com/auth/my-account/help) 통합을 위해 화이트리스트에 추가할 IP 세트를 제공할 수 있습니다.

### Data Ownership

우리의 WordPress 마이그레이션에서는 새로운 댓글 데이터나 업데이트된 댓글 데이터가 자동으로 백그라운드에서 귀하의 WordPress 설치에 동기화됩니다. 이는 댓글이 FastComments 자체에서 제공되어 WordPress 배포에 대한 부하를 줄이는 동시에, **우리는 또한** 백업용으로 데이터베이스에 저장한다는 의미입니다. 또한 FastComments를 다른 서비스로 전환하고자 할 경우, 귀하의 데이터는 이미 마이그레이션되어 최신 상태임을 의미합니다.