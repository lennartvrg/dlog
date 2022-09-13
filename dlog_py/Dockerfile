FROM arm32v7/rust

RUN apt-get -y update
RUN apt-get -y install python3
RUN apt-get -y install python3-pip 
RUN pip3 install -U maturin

ENTRYPOINT tail -f /dev/null
CMD [ "/bin/bash" ]