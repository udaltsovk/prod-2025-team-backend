package compression

import (
	"io"

	"github.com/klauspost/compress/zstd"
	"google.golang.org/grpc/encoding"
)

const ZstdCompression = "zstd"

// ZstdCompressor — экспортируемая структура для использования в других пакетах
type ZstdCompressor struct{}

// Compress — реализация сжатия Zstd
func (ZstdCompressor) Compress(w io.Writer) (io.WriteCloser, error) {
	encoder, err := zstd.NewWriter(w)
	if err != nil {
		return nil, err
	}
	return encoder, nil
}

// Decompress — реализация разжатия Zstd
func (ZstdCompressor) Decompress(r io.Reader) (io.Reader, error) {
	decoder, err := zstd.NewReader(r)
	if err != nil {
		return nil, err
	}
	return decoder, nil
}

// Name возвращает название алгоритма
func (ZstdCompressor) Name() string {
	return ZstdCompression
}

func init() {
	encoding.RegisterCompressor(ZstdCompressor{})
}
