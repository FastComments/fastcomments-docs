### Maven

프로젝트의 POM에 Repsy 저장소를 추가합니다:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

그 다음 필요한 의존성을 추가합니다:

```xml
<dependencies>
    <!-- API 클라이언트 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- 코어 라이브러리 (SSO 포함) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub 라이브러리 (실시간 이벤트용) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

build.gradle 파일에 Repsy 저장소를 추가합니다:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API 클라이언트
    implementation "com.fastcomments:client:3.1.1"
    
    // 코어 라이브러리 (SSO 포함)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub 라이브러리 (실시간 이벤트용)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### 라이브러리 구성

이 라이브러리는 세 개의 모듈로 구성되어 있습니다. 생성된 API 클라이언트, API 사용을 더 쉽게 만들어주는 직접 작성된 유틸리티를 포함하는 코어 Java 라이브러리, 그리고 변경 피드를 구독하기 위한 라이브러리인 `pubsub` 모듈입니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [SSO 예제를 포함한 코어 라이브러리 문서](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 라이브러리 문서](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 공개 API vs 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 가지 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하며, `PublicApi`는 인증 없이 브라우저/모바일 기기 등에서 직접 호출할 수 있는 메서드를 포함합니다.

`ModerationApi`는 광범위한 실시간 고속 검토(모더레이션) API 모음을 제공합니다. 모든 `ModerationApi` 메서드는 `sso` 파라미터를 받으며, SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.