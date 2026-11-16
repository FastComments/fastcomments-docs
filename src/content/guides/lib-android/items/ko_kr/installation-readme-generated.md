앱의 `build.gradle.kts` 파일에 FastComments SDK를 추가하세요:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

프로젝트의 `settings.gradle.kts`에 Repsy 저장소가 있는지 확인하세요:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // 기타 저장소...
    }
}
```