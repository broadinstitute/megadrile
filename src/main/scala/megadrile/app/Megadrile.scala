package megadrile.app

import better.files.File
import htsjdk.variant.vcf.VCFFileReader
import scala.jdk.CollectionConverters.{ IteratorHasAsScala, ListHasAsScala}

object Megadrile {
  def main(args: Array[String]): Unit = {
    val dataDir = File("/mnt/c/Users/oliverr/kg")
    val dataFile =
      dataDir / "ALL.chr22.phase3_shapeit2_mvncall_integrated_v5a.20130502.genotypes.vcf.gz"
    val vcfReader = new VCFFileReader(dataFile.path)
    val vcfHeader = vcfReader.getHeader
    val nSamples = vcfHeader.getNGenotypeSamples
    println("Number of genotype samples: " + nSamples)
    println("Now reading genotypes")
    for(variantContext <- vcfReader.iterator().asScala.take(20)) {
      val chrom = variantContext.getContig
      val pos = variantContext.getStart
      val refAllele = variantContext.getReference
      val ref = refAllele.getBaseString
      val altAlleles = variantContext.getAlternateAlleles.asScala
      for(altAllele <- altAlleles) {
        val alt = altAllele.getBaseString
        println(s"$chrom:$pos:$ref:$alt\t${refAllele.isSymbolic}\t${altAllele.isSymbolic}")
        val genotypeContext = variantContext.getGenotypes
        for(genotype <- genotypeContext.iterator().asScala) {
          val dose = genotype.countAllele(altAllele)
        }
      }
    }
    println("Done!")
  }
}
