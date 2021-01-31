require 'vagrant-openstack-provider'

 
Vagrant.configure('2') do |config|

  #SSH forward port 22 -> 2222
  config.vm.network "forwarded_port", guest: 22, host:2222, id: "ssh", auto_correct: true

  #Expose port 80 -> 8080 for web Service (future)
  config.vm.network "forwarded_port", guest: 443, host:443, id: "websecure", auto_correct: true
  
  #Expose port 3030 -> 3030 for api Service (future)
  config.vm.network "forwarded_port", guest: 3030, host:3030, id: "service1", auto_correct: true

  #Expose port 3031 -> 3031 for api Service (future)
  config.vm.network "forwarded_port", guest: 3031, host:3031, id: "service2", auto_correct: true

  #Expose port 3032 -> 3032 for api Service (future)
  config.vm.network "forwarded_port", guest: 3032, host:3032, id: "service3", auto_correct: true

  #Expose port 3033 -> 3033 for api Service (future)
  config.vm.network "forwarded_port", guest: 3033, host:3033, id: "service4", auto_correct: true

  
  config.vm.define :'vagrant-ubuntu' do |v|
 
    v.ssh.username = 'ubuntu'
    v.ssh.private_key_path = '~/.ssh/ovh'
    v.ssh.insert_key = 'false'
    v.vm.synced_folder '.', '/vagrant', type: 'rsync', disabled: true
    v.vm.provider :openstack do |provider|
      provider.openstack_auth_url    = ENV['OS_AUTH_URL']
      provider.openstack_network_url = 'https://network.compute.uk1.cloud.ovh.net/v2.0'
      provider.identity_api_version  = ENV['OS_IDENTITY_API_VERSION']
      provider.username              = ENV['OS_USERNAME']
      provider.password              = ENV['OS_PASSWORD']
      provider.domain_name           = ENV['OS_USER_DOMAIN_NAME']
      provider.project_name          = ENV['OS_PROJECT_NAME']
      provider.tenant_name           = ENV['OS_TENANT_NAME']
      provider.project_domain_name   = ENV['OS_USER_DOMAIN_NAME']
      provider.flavor                = 's1-4'
      provider.image                 = 'Ubuntu 18.04'
      provider.keypair_name          = 'ovh'
      provider.region                = ENV['OS_REGION_NAME']
      provider.networks              = [ 'Ext-Net' ]
    end
  end

  config.vm.provision :ansible do |ansible|
    ansible.limit = "all"
    ansible.playbook = "provision.yaml"
    # ansible.verbose = "v"
  end
end