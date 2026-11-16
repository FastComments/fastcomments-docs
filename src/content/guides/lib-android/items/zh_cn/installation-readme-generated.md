将 FastComments SDK 添加到您应用的 `build.gradle.kts` 文件中：

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

请确保您项目的 `settings.gradle.kts` 中包含 Repsy 仓库：

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // 其他仓库...
    }
}
```