将 FastComments SDK 添加到您应用的 `build.gradle.kts` 文件：

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

确保您的项目的 `settings.gradle.kts` 中包含 Repsy 仓库：

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```