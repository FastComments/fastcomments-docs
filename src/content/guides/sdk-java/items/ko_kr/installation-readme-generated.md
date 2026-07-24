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

그런 다음 필요한 종속성을 추가합니다:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
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
    // API Client
    implementation "com.fastcomments:client:3.0.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### 라이브러리 내용

이 라이브러리는 세 개의 모듈을 포함합니다. 생성된 API 클라이언트, API 사용을 더 쉽게 해주는 수작업 유틸리티를 포함한 핵심 Java 라이브러리, 그리고 변경 피드를 구독하기 위한 `pubsub` 모듈입니다.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 공개 API와 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 개의 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 브라우저/모바일 기기 등에서 인증 없이 직접 호출할 수 있는 메서드를 포함합니다.

`ModerationApi`는 실시간 및 빠른 검토를 위한 광범위한 API 세트를 제공합니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 받아들이며, SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.