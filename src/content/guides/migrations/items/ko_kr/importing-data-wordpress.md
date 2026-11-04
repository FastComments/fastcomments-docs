저희 [WordPress 플러그인](https://wordpress.org/plugins/fastcomments/)은 강력한 UI 기반 가져오기 메커니즘을 제공합니다. 플러그인을 설치하면 WordPress 설치 환경을 FastComments와 연결하고 기존 댓글 데이터를 복사하는 과정을 안내해 드립니다.

**이 과정은 아무것도 수동으로 복사하거나 다운로드할 필요 없이 이루어집니다.**

마이그레이션 과정은 마이그레이션 중 UI를 통해 표시됩니다. 대부분의 마이그레이션은 몇 분 안에 완료됩니다.

이 메커니즘은 마이그레이션 중에 WordPress 설치 환경에 과도한 부하를 주지 않도록 설계되었습니다.

WordPress에서 사이트를 이전하는 경우, 플러그인을 사용하는 대신 WordPress XML 또는 CSV 내보내기를 가져올 수 있습니다. [댓글을 새 사이트로 이전하기](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)를 참조하세요.

### CloudFlare 및 방화벽

자동화된 WordPress 설정이 작동하려면 WordPress 설치 환경에 호출을 해야 합니다. Cloudflare와 같은 방화벽이 저희를 차단하여 연동이 실패할 수 있습니다. 이러한 경우, 연동을 위해 [저희가](https://fastcomments.com/auth/my-account/help) 화이트리스트에 추가할 IP 목록을 제공해 드릴 수 있습니다.

### 데이터 소유권

WordPress 마이그레이션의 경우, 새로운 댓글 데이터나 업데이트된 댓글 데이터는 자동으로 백그라운드에서 WordPress 설치 환경으로 동기화됩니다. 즉, 댓글은 FastComments가 직접 제공하여 WordPress 배포 환경의 부하를 줄이는 동시에, 백업으로 **또한** 데이터베이스에 저장됩니다. 이는 나중에 FastComments를 떠나고 싶으실 때 데이터가 이미 마이그레이션되어 최신 상태로 유지된다는 의미이기도 합니다.