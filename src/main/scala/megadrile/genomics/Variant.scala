package megadrile.genomics

final case class Variant(chrom: String, pos: Int, ref: String, alt: String) {
  override def toString: String = s"$chrom:$pos:$ref:$alt"
}
