import threading
import subprocess

#write_lock = threading.Lock()
#
#class Resolver (threading.Thread):
#   def _init_(self, threadID, name, domain):
#       threading.Thread.__init__(self)
#       self.threadID = threadID
#       self.name = name
#       self.domain = domain
#   def run(self):
#       write_lock.acquire()
#       print "Starting resolver " + self.name + " for " + self.domain
#       write_lock.release()
#       # Find the IPs for this domain!
#       p = subprocess.Popen (['/usr/bin/dig', '+short', self.domain],
#                               stdout = subprocess.PIPE,
#                               stderr = subprocess.PIPE)
#
#print "wtf"
#
#thread = Resolver (1, "Name-test1", "test1.ro")
#thread.start()
#thread = Resolver (2, "Name-test2", "test2.ro" )
#thread.start()

p = subprocess.Popen (  ['/usr/bin/dig', '+short', self.domain],.
                         stdout = subprocess.PIPE,
                         stderr = subprocess.PIPE)

out, err = p.communicate ()
ip_list = out.split("\n")
print out
