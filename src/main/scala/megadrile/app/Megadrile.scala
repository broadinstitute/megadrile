package megadrile.app

import better.files.File
import htsjdk.variant.vcf.VCFFileReader
import megadrile.genomics.Variant
import megadrile.stats.{SlidingWindowStats, VariantStats}

import scala.jdk.CollectionConverters.{IteratorHasAsScala, ListHasAsScala}

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
    val windowSize = 1000000
    val slidingWindowStats =
      new SlidingWindowStats(windowSize, SlidingWindowStats.Consumer.NoOpConsumer)
    for(variantContext <- vcfReader.iterator().asScala.take(5000)) {
      val chrom = variantContext.getContig
      val pos = variantContext.getStart
      val refAllele = variantContext.getReference
      val ref = refAllele.getBaseString
      val altAlleles = variantContext.getAlternateAlleles.asScala
      for(altAllele <- altAlleles) {
        val alt = altAllele.getBaseString
        val variant = Variant(chrom, pos, ref, alt)
        val variantStats = VariantStats.create(variantContext, altAllele, nSamples)
        slidingWindowStats.addVariantStats(variant, variantStats)
      }
    }
    slidingWindowStats.flushAll()
    println("Done! Read " + slidingWindowStats.nVariants + " variants.")
  }
}
