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
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
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
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### 라이브러리 구성

이 라이브러리는 세 개의 모듈로 구성됩니다. 생성된 API 클라이언트, API 작업을 더 쉽게 해주는 수작업 유틸리티를 포함한 코어 Java 라이브러리, 그리고 변경 피드 구독을 위한 `pubsub` 모듈입니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [코어 라이브러리 문서 (SSO 예제 포함)](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 라이브러리 문서](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 공개 API vs 보안 API

API 클라이언트에는 `DefaultApi`와 `PublicApi` 두 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 인증 없이 브라우저/모바일 기기 등에서 직접 호출할 수 있는 API 호출을 포함합니다.