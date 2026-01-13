앱의 `build.gradle.kts` 파일에 FastComments SDK를 추가하세요:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

프로젝트의 `settings.gradle.kts` 파일에 Repsy 리포지토리가 포함되어 있는지 확인하세요:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // 다른 리포지토리...
    }
}
```