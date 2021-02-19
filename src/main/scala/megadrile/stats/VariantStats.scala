package megadrile.stats

import htsjdk.variant.variantcontext.{Allele, VariantContext}
import scala.jdk.CollectionConverters.IteratorHasAsScala

final class VariantStats(
                          val refAllele: Allele,
                          val altAllele: Allele,
                          val nSamples: Int,
                          val dosageStore: DosageStore,
                          val dosageSum: Double,
                          val dosageSquaredSum: Double
                        ) {
  def mean: Double = dosageSum / nSamples

  def variance: Double = dosageSquaredSum / nSamples

  def sigma: Double = Math.sqrt(variance)

  def altAlleleFrequency: Double = mean / VariantStats.ploidy

  def minorAlleleFrequency: Double = {
    val altF = altAlleleFrequency
    if (altF <= 0.5) altF else 1.0 - altF
  }

  def covariance(thatVariantStats: VariantStats): Double = {
    var productSum: Double = 0.0
    for (iSample <- 0 to nSamples) {
      productSum += dosageStore.getDosage(iSample) * thatVariantStats.dosageStore.getDosage(iSample)
    }
    val productMean = productSum / nSamples
    productMean - mean * thatVariantStats.mean
  }

  def correlation(thatVariantStats: VariantStats): Double = {
    covariance(thatVariantStats) / Math.sqrt(variance * thatVariantStats.variance)
  }
}

object VariantStats {
  val ploidy: Int = 2

  def create(variantContext: VariantContext, altAllele: Allele, nSamples: Int): VariantStats = {
    val refAllele = variantContext.getReference
    val dosageStore = DosageStore(nSamples)
    var iGenotype: Int = 0
    var dosageSum: Double = 0.0
    var dosageSquareSum: Double = 0.0
    for (genotype <- variantContext.getGenotypes.iterator().asScala) {
      val dosage = genotype.countAllele(altAllele)
      dosageStore.setDosage(iGenotype, dosage.toByte)
      dosageSum += dosage
      dosageSquareSum += dosage * dosage
      iGenotype += 1
    }
    new VariantStats(refAllele, altAllele, nSamples, dosageStore, dosageSum, dosageSquareSum)
  }
}
