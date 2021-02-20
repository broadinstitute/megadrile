import sbt.project

val megadrileV = "0.0.1"
val scalaV = "2.13.4"
val htsjdkV = "2.24.0"
val yootilzV = "0.1.5"
val scalaTestV = "3.2.2"
val betterFilesV = "3.9.1"
val googleCloudNioV = "0.122.6"
val scallopV = "4.0.2"
val configV = "1.4.1"

lazy val mainDeps = Seq(
  "com.github.samtools" % "htsjdk" % htsjdkV,
  "org.broadinstitute" %% "yootilz-core" % yootilzV,
  "org.broadinstitute" %% "yootilz-gcp" % yootilzV,
  "com.github.pathikrit" %% "better-files" % betterFilesV,
  "com.google.cloud" % "google-cloud-nio" % googleCloudNioV,
  "org.rogach" %% "scallop" % scallopV,
  "com.typesafe" % "config" % configV
)

lazy val testDeps = Set(
  "org.scalatest" %% "scalatest" % scalaTestV % "test"
)

lazy val root = (project in file("."))
  .settings(
    name := "megadrile",
    name in Linux := "megadrile",
    version := megadrileV,
    scalaVersion := scalaV,
    libraryDependencies ++= (mainDeps ++ testDeps),
    scalacOptions ++= Seq("-feature", "-deprecation", "-unchecked"),
    mainClass in (Compile, run) := Some("megadrile.app.Megadrile"),
    maintainer := "Oliver A Ruebenacker <oliverr@broadinstitute.org>",
    packageSummary := "Crunching genomic stats from VCF file",
    packageDescription := "Crunching genomic statistics from VCF file, such as covariances",
    debianPackageDependencies := Seq("java8-runtime-headless"),
    debianNativeBuildOptions in Debian := Seq("-Zgzip", "-z3") // gzip compression at level 3
  ).enablePlugins(JavaAppPackaging, DebianPlugin)

