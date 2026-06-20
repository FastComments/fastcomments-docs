### Maven

프로젝트의 POM에 Repsy 저장소를 추가하세요:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

그런 다음 필요한 의존성을 추가하세요:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

build.gradle 파일에 Repsy 저장소를 추가하세요:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### 라이브러리 구성

이 라이브러리는 세 개의 모듈을 포함합니다. 생성된 API 클라이언트, API 사용을 더 쉽게 해주는 수작업 유틸리티를 포함하는 코어 Java 라이브러리, 그리고 변경 피드를 구독하기 위한 `pubsub` 모듈입니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core 라이브러리 문서(SSO 예제 포함)](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 라이브러리 문서](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 공개 API vs 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 가지 클래스가 있습니다. `DefaultApi`에는 API 키가 필요한 메서드들이 포함되어 있고, `PublicApi`에는 브라우저/모바일 기기 등에서 인증 없이 직접 호출할 수 있는 메서드들이 포함되어 있습니다.

`ModerationApi`는 운영자 대시보드를 구동합니다. 댓글 관리(목록, 카운트, 검색, 로그, 내보내기), 관리 작업(제거/복원, 신고, 검토/스팸/승인 상태 설정, 투표, 스레드 재개/종료), 차단(댓글 차단, 차단 해제, 사전 차단 요약, 차단 상태 및 설정, 차단된 사용자 수) 및 배지와 신뢰도(배지 수여/제거, 수동 배지, 신뢰도 조회/설정, 사용자 내부 프로필)를 위한 메서드를 포함합니다. 모든 `ModerationApi` 메서드는 `sso` 파라미터를 허용하므로 호출을 SSO로 인증된 운영자를 대신하여 수행할 수 있습니다.