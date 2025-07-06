#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"
#include <libheif/heif.h>
#include <iostream>
#include <cstring>
#include <string>
using namespace std;

extern "C"
{
int to_heif(const char* input_file)
{
    int width, height, channels;
    unsigned char* data = stbi_load(input_file, &width, &height, &channels, 3);
    if (!data) {
        std::cerr << "stbi_load falhou para: " << input_file << std::endl;
        return -1;
    }

    heif_context* ctx = heif_context_alloc();
    if (!ctx) {
        std::cerr << "heif_context_alloc falhou" << std::endl;
        stbi_image_free(data);
        return -1;
    }

    heif_image* image;
    heif_error err = heif_image_create(width, height, heif_colorspace_RGB, heif_chroma_interleaved_RGB, &image);
    if (err.code != heif_error_Ok) {
        std::cerr << "heif_image_create falhou: " << err.message << std::endl;
        heif_context_free(ctx);
        stbi_image_free(data);
        return -1;
    }

    int stride = 0;
    uint8_t* dst_data = heif_image_get_plane(image, heif_channel_interleaved, &stride);
    if (!dst_data) {
        std::cerr << "heif_image_get_plane falhou" << std::endl;
        heif_image_release(image);
        heif_context_free(ctx);
        stbi_image_free(data);
        return -1;
    }

    for (int y = 0; y < height; y++) {
        memcpy(dst_data + y * stride, data + y * width * 3, width * 3);
    }

    heif_encoder* encoder;
    err = heif_context_get_encoder_for_format(ctx, heif_compression_HEVC, &encoder);
    if (err.code != heif_error_Ok) {
        std::cerr << "heif_context_get_encoder_for_format falhou: " << err.message << std::endl;
        heif_image_release(image);
        heif_context_free(ctx);
        stbi_image_free(data);
        return -1;
    }

    heif_encoder_set_lossy_quality(encoder, 80);

    heif_image_handle* handle;
    err = heif_context_encode_image(ctx, image, encoder, nullptr, &handle);
    if (err.code != heif_error_Ok) {
        std::cerr << "heif_context_encode_image falhou: " << err.message << std::endl;
        heif_encoder_release(encoder);
        heif_image_release(image);
        heif_context_free(ctx);
        stbi_image_free(data);
        return -1;
    }

    std::string output_file = std::string(input_file) + ".heif";
    err = heif_context_write_to_file(ctx, output_file.c_str());
    if (err.code != heif_error_Ok) {
        std::cerr << "heif_context_write_to_file falhou: " << err.message << std::endl;
    }

    heif_image_release(image);
    heif_encoder_release(encoder);
    heif_image_handle_release(handle);
    heif_context_free(ctx);
    stbi_image_free(data);

    std::cout << "Imagem convertida com sucesso para " << output_file << std::endl;
    return 0;
}

struct heif_error heif_context_write_to_file(struct heif_context*, const char* filename);
}