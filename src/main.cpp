#include <cstdlib>
#include <fstream>
#include <iostream>
#include <filesystem>
#include <thread>
#include <chrono>
#include <cstring>

using namespace std;

extern "C"
{
int create_image(const char* img_path, int size_mb)
{
    ofstream ofs(img_path, ios::binary);
    if (!ofs.is_open())
        return -1;
    ofs.seekp(size_mb * 1024 * 1024 - 1);
    ofs.write("", 1);
    ofs.close();

    return 0;
}

// minimum --> 128 bytes
int attach_image(const char* img_path, char* device_buf, size_t buf_size)
{
    string cmd = "hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount ";
    cmd += img_path;

    FILE* pipe = popen(cmd.c_str(), "r");
    if (!pipe)
        return -1;

    if (!fgets(device_buf, static_cast<int>(buf_size), pipe))
    {
        pclose(pipe);
        return -1;
    }
    pclose(pipe);

    // Remove newline do device_buf
    size_t len = strlen(device_buf);
    if (len > 0 && device_buf[len-1] == '\n')
        device_buf[len-1] = '\0';

    return 0;
}

int format_apfs(const char* device, const char* volume_name)
{
    string cmd = "diskutil eraseDisk APFS ";
    cmd += volume_name;
    cmd += " ";
    cmd += device;

    int ret = system(cmd.c_str());
    if (ret != 0)
        return -1;
    return 0;
}

int write_image(const char* mount_point, const char* filename, const char* content)
{
    std::string path = std::string(mount_point) + "/" + filename;
    std::ofstream ofs(path, std::ios::out | std::ios::binary);
    if (!ofs)
        return -1;
    ofs << content;
    return 0;
}


int detach_image(const char* device)
{
    string cmd = "hdiutil detach ";
    cmd += device;
    int ret = system(cmd.c_str());
    if (ret != 0)
        return -1;
    return 0;
}
}