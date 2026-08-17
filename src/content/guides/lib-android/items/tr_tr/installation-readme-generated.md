FastComments SDK'sını uygulamanızın `build.gradle.kts` dosyasına ekleyin:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Projenizin `settings.gradle.kts` dosyasında Repsy deposunun bulunduğundan emin olun:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // diğer depolar...
    }
}
```