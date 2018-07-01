#!/usr/bin/bash

# create host and router
sudo ip netns add host1
sudo ip netns add host2
sudo ip netns add host3
sudo ip netns add RT
sudo ip netns add NR

# connect cables
sudo ip link add host1_veth0 type veth peer name RT_veth1
sudo ip link add host2_veth0 type veth peer name RT_veth2
sudo ip link add host3_veth0 type veth peer name RT_veth3
sudo ip link add RT_veth0 type veth peer name NR_veth1
sudo ip link add NR_veth0 type veth peer name Linux_veth1

# make interfaces belong to each namespace
sudo ip link set host1_veth0 netns host1
sudo ip link set host2_veth0 netns host2
sudo ip link set host3_veth0 netns host3
sudo ip link set RT_veth0 netns RT
sudo ip link set RT_veth1 netns RT
sudo ip link set RT_veth2 netns RT
sudo ip link set RT_veth3 netns RT
sudo ip link set NR_veth0 netns NR
sudo ip link set NR_veth1 netns NR

# assign IP address
sudo ip netns exec host1 ip addr add 192.168.0.1/24 dev host1_veth0
sudo ip netns exec host2 ip addr add 192.168.0.2/24 dev host2_veth0
sudo ip netns exec host3 ip addr add 192.168.1.1/24 dev host3_veth0
sudo ip netns exec RT ip addr add 192.168.0.254/24 dev RT_veth1
sudo ip netns exec RT ip addr add 192.168.0.253/24 dev RT_veth2
sudo ip netns exec RT ip addr add 192.168.1.254/24 dev RT_veth3
sudo ip netns exec RT ip addr add 192.168.128.1/24 dev RT_veth0
sudo ip netns exec NR ip addr add 192.168.128.254/24 dev NR_veth1
sudo ip netns exec NR ip addr add 192.168.129.1/24 dev NR_veth0
sudo ip addr add 192.168.129.254/24 dev Linux_veth1

# link up
sudo ip netns exec host1 ip link set lo up
sudo ip netns exec host2 ip link set lo up
sudo ip netns exec host3 ip link set lo up
sudo ip netns exec RT ip link set lo up
sudo ip netns exec NR ip link set lo up

sudo ip netns exec host1 ip link set host1_veth0 up
sudo ip netns exec host2 ip link set host2_veth0 up
sudo ip netns exec host3 ip link set host3_veth0 up
sudo ip netns exec RT ip link set RT_veth0 up
sudo ip netns exec RT ip link set RT_veth1 up
sudo ip netns exec RT ip link set RT_veth2 up
sudo ip netns exec RT ip link set RT_veth3 up
sudo ip netns exec NR ip link set NR_veth1 up
sudo ip netns exec NR ip link set NR_veth0 up
sudo ip link set Linux_veth1 up
