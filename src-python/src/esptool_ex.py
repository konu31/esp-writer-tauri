import serial.tools.list_ports
import json
import esptool
import sys
import os


def is_debug():
    return hasattr(sys, "_MEIPASS") == False


def app_path():
    return os.path.abspath(".") if is_debug() else sys._MEIPASS


def get_firmware_file_path(filename):
    firmware_dir = os.path.join(app_path(), "resources", "firmware") if is_debug() \
        else os.path.join(app_path(), "esptool", "firmware")
    
    return os.path.join(firmware_dir, filename)


def get_port_list():
    ports = serial.tools.list_ports.comports()
    port_list = [port.device for port in ports]
    print(json.dumps(port_list))


def get_board_info(port):
    args = [
        "-p", port,
        "flash_id"
    ]
    esptool.main(args)


def write_firmware(port, chip, baudrate):
    args = [
        "--chip", chip,
        "--port", port,
        "--baud", baudrate,
        "--before", "default_reset", 
        "--after", "hard_reset", "write_flash", "-z",
        "--flash_mode", "dio",
        # "--flash_freq", "80m",
        "--flash_freq", "keep",
        "--flash_size", "detect"
    ]

    json_path = os.path.join(app_path(), "config.json")
    with open(json_path, 'r') as f:
        config = json.load(f)

    firmware_file_names = config["file"]
    address_list = config["address"][chip]
    for key in address_list:
        file_path = get_firmware_file_path(firmware_file_names[key])
        address = address_list[key]
        # 引数追加(ファームウェアファイル)
        args.append(address)
        args.append(file_path)

    # print(args)
    esptool.main(args)


if __name__ == "__main__":
    args = sys.argv
    mode = args[1]

    match mode:
        case "port_list":
            get_port_list()

        case "board_info":
            port = args[2]
            get_board_info(port)

        case "write_firmware":
            port = args[2]
            chip = args[3]
            baudrate = args[4]
            write_firmware(port, chip, baudrate)
