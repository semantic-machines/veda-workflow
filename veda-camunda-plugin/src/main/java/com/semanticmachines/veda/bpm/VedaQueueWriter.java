package com.semanticmachines.veda.bpm;

import org.sm.vqueue.VQueue;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Properties;

public class VedaQueueWriter {
  
  private static VedaQueueWriter instance = null;
  public VQueue queue;
  private static final Properties properties = new Properties();
  private static String queue_path;

  protected VedaQueueWriter() {
    loadProperties();
    try {
      queue = new VQueue();
      queue.setNameAndPath("camunda-event", queue_path);
    } catch (Exception e) {
      e.printStackTrace();
    }
  }

  public static VedaQueueWriter getInstance() {
    if (instance == null) {
      instance = new VedaQueueWriter();
    }
    return instance;
  }

  private static void writeDefaultProperties() {
    System.out.println("Writing default queue properties");
    queue_path = "./data/queue/camunda-events";
    properties.setProperty("queue_path", queue_path);
    FileOutputStream fos = null;
    try {
      fos = new FileOutputStream("camunda-veda-plugin.properties");
      properties.store(fos, null);
    } catch (IOException e) {
      e.printStackTrace();
    } finally {
      try {
    	if (fos != null) fos.close();
      } catch (IOException e) {
    	e.printStackTrace();
      }
    }
  }
  
  private static void loadProperties() {
	FileInputStream fis = null;
    try {
      fis = new FileInputStream("camunda-veda-plugin.properties");
      properties.load(fis);
      queue_path = properties.getProperty("queue_path", "");
      System.out.println(properties);
    } catch (IOException e) {
      writeDefaultProperties();
    } finally {
	  try {
		if (fis != null) fis.close();  
	  } catch (IOException e) {
		  e.printStackTrace();
	  }
    }
  }
  
}
