#!/usr/bin/env python3
"""Use this tool to generate ALL modules and memory mappings from an
SVD file for the lpc4337. The tool was designed to workaround most
issues, but I did not fix the problem where two registers may have the
same Field names, and it may generate a duplicated macro that will fail
to compile. Simply rename the duplicated fields. There weren't that many
but there were a few extracted from the SVD.
This was originally written for Python 2 but it's been updated to Python 3.
It internally uses svd2regs.py


REQUIRES:
pip install cmsis-svd pydentifier

USAGE:
python generate_lpc43xx_regs.py --svd LPC43xx_43Sxx.svd --destination ../../../chips/lpc43xx/src --cmd-path ../../../tools

"""
from xml.etree import ElementTree as ET
import argparse
import sys
import os
import subprocess

class FullPaths(argparse.Action):
    """Expand user- and relative-paths"""
    def __call__(self, parser, namespace, values, option_string=None):
        setattr(namespace, self.dest, os.path.abspath(os.path.expanduser(values)))

def is_dir(dirname):
    """Checks if a path is an actual directory"""
    if not os.path.isdir(dirname):
        msg = "{0} is not a directory".format(dirname)
        raise argparse.ArgumentTypeError(msg)
    else:
        return dirname

def prettty_print_args(args): 
    print(' '.join(str(p) for p in args) )

def get_destination_asb_path(destination, group_name):
    return destination + '/' + group_name.lower() + '.rs'

def process_peripheral(destination, cmd_path, svd, current_name, group_name, processed_names, literal):
    is_group = False
    args = ["python", cmd_path+"/svd2regs.py", "--svd",
                os.path.abspath(svd.name)]
    if group_name in processed_names:
        return ''
    # check if it needs to be grouped
    if current_name != group_name:
        args.append("-g")
        is_group = True
    filename = get_destination_asb_path(destination, group_name)
    args.extend(["--save", filename])
    if is_group:
        args.append(group_name)
    else:
        args.append(current_name)
    # call svd2regs and save to a new file
    prettty_print_args(args)
    p_output = subprocess.check_output(args)
    print(p_output)
    return p_output
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--svd', type=argparse.FileType('r'), const=sys.stdin,
                     nargs="?", metavar="SVD", help='Path to SVD-File', required=True)
    parser.add_argument('--destination', help='Path where the .rs files will be stored', action=FullPaths, type=is_dir, required=True)
    parser.add_argument('--cmd-path', help='full path where svd2regs.py is stored', action=FullPaths, type=is_dir, required=True)
    args = parser.parse_args()
    device = ET.parse(args.svd).getroot()
    processed_names = []
    for peripheral in device.find('peripherals').findall('peripheral'):
        print("#######################################")
        current_name = peripheral.find('name').text
        print('############ Finding group name for %s ############' % current_name)
        if peripheral.find('groupName') is not None:
            group_name = peripheral.find('groupName').text
        else:
            if current_name[:-1] in processed_names:
                print("WARNING: No group name, and apparently we already processed the group. SKIPPING")
                continue # if we successfully parsed the group, don't parse by peripheral again
            else:
                print("WARNING: using component name as group name. The SVD was funky.")
                group_name = current_name
        print('############ %s : %s ############' % (current_name, group_name))
        successful = False
        p_output = process_peripheral(args.destination, args.cmd_path, args.svd, current_name, group_name, processed_names, False)
        if 'Error: no peripheral found.' in p_output.decode("utf-8"):
            print("WARNING: svd2regs could not parse this as a group. retrying as a single component")
            os.remove(get_destination_asb_path(args.destination, group_name))
            p_output = process_peripheral(args.destination, args.cmd_path, args.svd, current_name, current_name, processed_names, False)
            if 'Error: no peripheral found.' not in p_output.decode("utf-8"):
                group_name = current_name
                successful = True
        else:
            successful = True
        if successful and group_name not in processed_names:
            processed_names.append(group_name)
    print("#######################################")

